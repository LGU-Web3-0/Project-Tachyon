//! This is our builtin frontend component. We don't use famous framework. Instead we use
//! yarn and gulp as our buider. We build typescript, tailwind css, and html5 with tools like webpack ... on our own
//! Here is a brief introduction on these tools:
//! * What is JavaScript? A Brief History
//!
//!JavaScript (also known as ECMAScript) started its life as a simple scripting language for browsers. At the time it was invented, it was expected to be used for short snippets of code embedded in a web page — writing more than a few dozen lines of code would have been somewhat unusual. Due to this, early web browsers executed such code pretty slowly. Over time, though, JS became more and more popular, and web developers started using it to create interactive experiences.
//!
//!Web browser developers responded to this increased JS usage by optimizing their execution engines (dynamic compilation) and extending what could be done with it (adding APIs), which in turn made web developers use it even more. On modern websites, your browser is frequently running applications that span hundreds of thousands of lines of code. This is long and gradual growth of “the web”, starting as a simple network of static pages, and evolving into a platform for rich applications of all kinds.
//!
//!More than this, JS has become popular enough to be used outside the context of browsers, such as implementing JS servers using node.js. The “run anywhere” nature of JS makes it an attractive choice for cross-platform development. There are many developers these days that use only JavaScript to program their entire stack!
//!
//!To summarize, we have a language that was designed for quick uses, and then grew to a full-fledged tool to write applications with millions of lines. Every language has its own quirks — oddities and surprises, and JavaScript’s humble beginning makes it have many of these. Some examples:
//!
//!TypeScript: A Static Type Checker
//!We said earlier that some languages wouldn’t allow those buggy programs to run at all. Detecting errors in code without running it is referred to as static checking. Determining what’s an error and what’s not based on the kinds of values being operated on is known as static type checking.
//!
//!
//!* Tailwind: <a href="https://tailwindcss.com/">tailwind</a>
//!
//!* webpack: webpack is a module bundler, the example script might like this:
//!```JavaScript
//!// This means webpack takes modules with dependencies
//!//   and emits static assets representing those modules.
//!
//!// dependencies can be written in CommonJs
//!var commonjs = require("./commonjs");
//!// or in AMD
//!define(["amd-module", "../file"], function(amdModule, file) {
//!	// while previous constructs are sync
//!	// this is async
//!	require(["big-module/big/file"], function(big) {
//!		 // for async dependencies webpack splits
//!		 //  your application into multiple "chunks".
//!		 // This part of your application is
//!		 //  loaded on demand (Code Splitting)
//!		var stuff = require("../my/stuff");
//!		// "../my/stuff" is also loaded on demand
//!		//  because it's in the callback function
//!		//  of the AMD require
//!	});
//!});
//!
//!
//!require("coffee!./cup.coffee");
//!// "Loaders" can be used to preprocess files.
//!// They can be prefixed in the require call
//!//  or configured in the configuration.
//!require("./cup");
//!// This does the same when you add ".coffee" to the extensions
//!//  and configure the "coffee" loader for /\.coffee$/
//!
//!
//!function loadTemplate(name) {
//!	return require("./templates/" + name + ".jade");
//!	// many expressions are supported in require calls
//!	// a clever parser extracts information and concludes
//!	//  that everything in "./templates" that matches
//!	//  /\.jade$/ should be included in the bundle, as it
//!	//  can be required.
//!}
//!
//!
//!// ... and you can combine everything
//!function loadTemplateAsync(name, callback) {
//!	require(["bundle?lazy!./templates/" + name + ".jade"],
//!	  function(templateBundle) {
//!		templateBundle(callback);
//!	});
//!}
//!```

include!("../.tmp/summary.rs");
