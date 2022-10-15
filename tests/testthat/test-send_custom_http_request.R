test_that("send_custom_http_request()", {
  get_request <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/get",
    "GET",
    list(),
    list(),
    list(),
    "")
  )
  expect_equal(get_request$status_code, "200 OK")

  post_request <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/post",
    "POST",
    list(),
    list(),
    list(),
    "")
  )
  expect_equal(post_request$status_code, "200 OK")

  put_request <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/put",
    "PUT",
    list(),
    list(),
    list(),
    "")
  )
  expect_equal(put_request$status_code, "200 OK")

  delete_request <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/delete",
    "DELETE",
    list(),
    list(),
    list(),
    "")
  )
  expect_equal(delete_request$status_code, "200 OK")

  header_request <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/headers",
    "GET",
    list(test_1 = "Hallo", test_2 = "Hallo 1", test_3 = "Hallo 3"),
    list(),
    list(),
    "")
  )
  expect_equal(header_request$status_code, "200 OK")

  query_params <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/get",
    "GET",
    list(),
    list(test_1 = "Hallo", test_2 = "Hallo 1", test_3 = "Hallo 3"),
    list(),
    "")
  )
  expect_equal(query_params$status_code, "200 OK")

  form_params <- jsonlite::fromJSON(send_custom_http_request(
    "https://httpbin.org/get",
    "GET",
    list(),
    list(),
    list(test_1 = "Hallo", test_2 = "Hallo 1", test_3 = "Hallo 3"),
    "")
  )
  expect_equal(form_params$status_code, "200 OK")

})
