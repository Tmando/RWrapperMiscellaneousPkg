% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\name{tuple_hash_v128_xof}
\alias{tuple_hash_v128_xof}
\title{This is a wrapper for \href{https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.TupleHashXof.html}{tuple_hash_v128_xof}}
\usage{
tuple_hash_v128_xof(custom_string, input)
}
\description{
This is a wrapper for \href{https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.TupleHashXof.html}{tuple_hash_v128_xof}
}
\examples{
tuple_hash_v128_xof(as.raw("Hallo Welt"),as.raw("Rust"))
}
