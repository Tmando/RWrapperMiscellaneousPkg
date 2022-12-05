#' return a HTTP Response
#' @export
#' @examples send_custom_http_request('https://httpbin.org/get','GET',list(),list(),list(),"")
#' send_custom_http_request('https://httpbin.org/post','POST',list(),list(),list(),"")
send_custom_http_request <- function(url, method, request_headers, request_query_params, request_form, request_body) {
    future::value(future::future(
        {
            return(.Call(wrap__send_custom_http_request, url, method, request_headers, request_query_params, request_form, request_body))
        },
        globals = list(url = url,method = method,request_headers = request_headers, request_query_params = request_query_params,request_form = request_form, request_body = request_body)
        )
    )
}