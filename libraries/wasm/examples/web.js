import("../pkg/index.js").then((identtiy) => {

    console.log(identtiy)

    const greet = identtiy.Greet()
    
    console.log("greet: ", greet)
});
