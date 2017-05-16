//! printpdf is a library designed for creating printable (PDF-X/3:2004 conform) PDF documents.
//! 
//! # Getting started
//!
//! ## Writing PDF
//! 
//! There are two types of functions: `add_*` and `use_*`. `add_*`-functions operate on the
//! document and return a reference to the content that has been added. This is used for 
//! instantiating objects via references in the document (for example, for reusing a block of 
//! data - like a font) without copying it (and bloating the file size).
//! 
//! Instancing happens via the `use_*`-functions, which operate on the layer. Meaning, you can only
//! instantiate blobs / content when you have a reference to the layer. Here are some examples:
//! 
//! ### Simple page
//! 
//! ```rust
//! use printpdf::*;
//! use std::fs::File;
//! use std::sync::Arc;
//! 
//! let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", 247.0, 210.0, "Layer 1");
//!
//! let mut output_file = File::create("test_simple_empty_file.pdf").unwrap();
//! 
//! Arc::try_unwrap(doc).unwrap().into_inner().unwrap().save(&mut output_file).unwrap();
//! ```
//! 
//! ### Adding pages
//! 
//! 
//! ### Adding fonts
//! 
//! 
//! # Useful links and resources
//! 
//! Resources I found while working on this library
//! 
//! [Official PDF 1.7 reference](http://www.adobe.com/content/dam/Adobe/en/devnet/acrobat/pdfs/pdf_reference_1-7.pdf)
//!
//! [[GERMAN] How to embed unicode fonts in PDF](http://www.p2501.ch/pdf-howto/typographie/vollzugriff/direkt)
//!
//! [PDF X/1-a Validator](https://www.pdf-online.com/osa/validate.aspx)
//!
//! [PDF X/3 technical notes](http://www.pdfxreport.com/lib/exe/fetch.php?media=en:technote_pdfx_checks.pdf)


#![allow(dead_code)]
#![feature(placement_in_syntax)]
#![feature(collection_placement)]
#![feature(custom_attribute)]
#![feature(drop_types_in_const)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] mod glob_macros;

extern crate lopdf;
extern crate freetype;
extern crate chrono;
extern crate rand;

pub mod traits;
pub mod types;
pub mod errors;
mod glob_defines;
mod indices;
#[cfg(test)] mod tests;

pub use self::traits::*;
pub use self::types::*;
pub use self::errors::*;
use glob_defines::*;
