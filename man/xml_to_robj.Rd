% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\name{xml_to_robj}
\alias{xml_to_robj}
\title{convert an xml string to json}
\usage{
xml_to_robj(xml_input_str)
}
\description{
This function converts an xml string to an robj
}
\examples{
inputStr <- "<dataset>
                   <record>
                       <id>1</id>
                        <first_name>Nerita</first_name>
                         <last_name>Stanney</last_name>
                         <email>nstanney0@domainmarket.com</email>
                         <gender>Female</gender>
                         <ip_address>223.10.217.33</ip_address>
                    </record>
       </dataset>"
xml_to_robj(inputStr)

}
