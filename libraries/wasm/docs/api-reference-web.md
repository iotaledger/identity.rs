## Classes

<dl>
<dt><a href="#DID">DID</a></dt>
<dd></dd>
<dt><a href="#Doc">Doc</a></dt>
<dd></dd>
<dt><a href="#Key">Key</a></dt>
<dd></dd>
<dt><a href="#NewDoc">NewDoc</a></dt>
<dd></dd>
<dt><a href="#PubKey">PubKey</a></dt>
<dd></dd>
<dt><a href="#VerifiableCredential">VerifiableCredential</a></dt>
<dd></dd>
<dt><a href="#VerifiablePresentation">VerifiablePresentation</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#publish">publish(doc, params)</a> ⇒ <code>any</code></dt>
<dd><p>Publishes a DID Document to the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#resolve">resolve(did, params)</a> ⇒ <code>any</code></dt>
<dd><p>Resolves the latest DID Document from the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#checkCredential">checkCredential(data, params)</a> ⇒ <code>any</code></dt>
<dd><p>Validates credential with the DID Document from the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#checkPresentation">checkPresentation(data, params)</a> ⇒ <code>any</code></dt>
<dd><p>Validates credential with the DID Document from the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#start">start()</a></dt>
<dd></dd>
<dt><a href="#initialize">initialize()</a> ⇒ <code>any</code></dt>
<dd><p>Initializes the console_error_panic_hook for better error messages</p>
</dd>
</dl>

<a name="NewDoc"></a>

## NewDoc
**Kind**: global class  

* [NewDoc](#NewDoc)
    * [.key](#NewDoc+key) ⇒ [<code>Key</code>](#Key)
    * [.doc](#NewDoc+doc) ⇒ [<code>Doc</code>](#Doc)

<a name="NewDoc+key"></a>

### newDoc.key ⇒ [<code>Key</code>](#Key)
**Kind**: instance property of [<code>NewDoc</code>](#NewDoc)  
<a name="NewDoc+doc"></a>

### newDoc.doc ⇒ [<code>Doc</code>](#Doc)
**Kind**: instance property of [<code>NewDoc</code>](#NewDoc)  
<a name="publish"></a>

## publish(doc, params) ⇒ <code>any</code>
Publishes a DID Document to the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| doc | <code>any</code> | 
| params | <code>any</code> | 

<a name="resolve"></a>

## resolve(did, params) ⇒ <code>any</code>
Resolves the latest DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| did | <code>string</code> | 
| params | <code>any</code> | 

<a name="checkCredential"></a>

## checkCredential(data, params) ⇒ <code>any</code>
Validates credential with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>string</code> | 
| params | <code>any</code> | 

<a name="checkPresentation"></a>

## checkPresentation(data, params) ⇒ <code>any</code>
Validates credential with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>string</code> | 
| params | <code>any</code> | 

<a name="start"></a>

## start()
**Kind**: global function  
<a name="initialize"></a>

## initialize() ⇒ <code>any</code>
Initializes the console_error_panic_hook for better error messages

**Kind**: global function  
