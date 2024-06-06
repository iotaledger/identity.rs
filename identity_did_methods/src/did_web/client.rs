use std::{fs::File, io::{self, Read}, net::SocketAddr, sync::Arc};
use hickory_resolver::{config::{ResolverConfig, ResolverOpts}, TokioAsyncResolver};
use once_cell::sync::OnceCell;
use reqwest::{dns::Addrs, Certificate, Client, ClientBuilder, RequestBuilder};

use crate::Error;
use crate::Result;

/// A `WebClientBuilder` (wrapper to `reqwest::ClientBuilder`) can be used to create a `WebClient` with custom configuration.
#[must_use]
pub struct WebClientBuilder{
    inner: ClientBuilder
}

impl WebClientBuilder {
    /// Constructs a new `WebClientBuilder`.
    /// This is the same as `WebClient::builder()`.
    pub fn new() -> Self {
        Self { inner: ClientBuilder::new() }
    }

    pub fn from(builder: ClientBuilder) -> Self {
        Self { inner: builder }
    }

    pub fn add_root_certificate_pem(self, cert_path: &str) -> Result<Self>{
        let mut buf = Vec::new();
        File::open(cert_path)
        .map_err(|_| Error::AddRootCertificateError)?
        .read_to_end(&mut buf)
        .map_err(|_| Error::AddRootCertificateError)?;

        let cert = Certificate::from_pem(&buf).map_err(Error::WebClientBuildError)?;
        Ok(Self{inner: self.inner.add_root_certificate(cert)})
    }

    pub fn add_root_certificate_der(self, cert_path: &str) -> Result<Self>{
        let mut buf = Vec::new();
        File::open(cert_path)
        .map_err(|_| Error::AddRootCertificateError)?
        .read_to_end(&mut buf)
        .map_err(|_| Error::AddRootCertificateError)?;

        let cert = Certificate::from_der(&buf).map_err(Error::WebClientBuildError)?;
        Ok(Self{inner: self.inner.add_root_certificate(cert)})
    }

    /// Returns a `WebClient` that uses this `WebClientBuilder` configuration.
    pub fn build(self) -> Result<WebClient> {
        let client = self.inner.build().map_err(|e| Error::WebClientBuildError(e) )?;
        Ok(WebClient{ inner: client })
    }
}

impl std::ops::Deref for WebClientBuilder {
    type Target = ClientBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for WebClientBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// An asynchronous `WebClient` to make Requests with.
/// The `WebClient` is a wrapper to `reqwest::Client`, so has various configuration values to tweak, but the defaults are set to what is usually the most commonly desired value for DID Web Resolve. 
/// To configure a `WebClient`, use `WebClient::builder()`.
#[derive(Clone)]
pub struct WebClient{
    inner: Client
}

impl WebClient {

    pub fn new(client: Client) -> Self {
        Self { inner: client }
    }

    /// Constructs a new `WebClient` with the defaul configuration using a DNS over HTTPS resolver.
    pub fn default() -> Result<Self> {
        // Construct a new Resolver with default configuration options
        let resolver = DnsOverHttpsResolver::default();

        let client = ClientBuilder::new()
        .use_rustls_tls()
        .dns_resolver(resolver.into())
        .build()
        .map_err(|e| Error::WebClientBuildError(e) )?;

        Ok(Self { inner: client })
    }

    /// Creates a `WebClientBuilder` to configure a `WebClient`.
    ///
    /// This is the same as `WebClientBuilder::new()`.
    pub fn builder() -> WebClientBuilder {
        WebClientBuilder::new()
    }

    /// Convenience method to make a GET request to a URL of type `identity_core::common::Url`.
    pub fn get(&self, url: identity_core::common::Url) -> RequestBuilder {
        self.inner.get(url.as_ref())
    }

}
  

impl std::ops::Deref for WebClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for WebClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}



/// DNS over HTTPS resolver, which implements the `reqwest::dns::Resolve` trait.
#[derive(Debug, Default, Clone)]
pub(crate) struct DnsOverHttpsResolver {
    /// Since we might not have been called in the context of a
    /// Tokio Runtime in initialization, so we must delay the actual
    /// construction of the resolver.
    state: Arc<OnceCell<TokioAsyncResolver>>,
}

impl DnsOverHttpsResolver {
    fn new_https_resolver()-> io::Result<TokioAsyncResolver> {
        let mut opt = ResolverOpts::default();
        opt.validate=true; 
        Ok(TokioAsyncResolver::tokio(
            ResolverConfig::cloudflare_https(),
        opt
        ))
    }
}

impl reqwest::dns::Resolve for DnsOverHttpsResolver {
    fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving {
        let resolver = self.clone();
        Box::pin(async move {
            let resolver = resolver.state.get_or_try_init(DnsOverHttpsResolver::new_https_resolver)?;
            let lookup = resolver.lookup_ip(name.as_str()).await?;
            let addrs: Addrs = Box::new(
                lookup.into_iter().map(|ip_addr| SocketAddr::new(ip_addr, 0))
            );
            Ok(addrs)
        })
    }
}
