% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\name{kangaroo_twelve}
\alias{kangaroo_twelve}
\title{This is a wrapper for \href{https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.KangarooTwelve.html}{kangaroo_twelve}}
\usage{
kangaroo_twelve(custom_string, input)
}
\description{
This is a wrapper for \href{https://docs.rs/tiny-keccak/latest/tiny_keccak/struct.KangarooTwelve.html}{kangaroo_twelve}
}
\examples{
kangaroo_twelve_xof(as.raw("Hallo Welt"),as.raw("Test"))
}
