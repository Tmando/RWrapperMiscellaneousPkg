test_that("add_num()", {
  expect_identical(add_num(3, 4), 7.0)
  expect_identical(add_num(1, 2), 3.0)
  expect_identical(add_num(4, 5), 9.0)
  expect_identical(add_num(10, 5), 15.0)
  expect_error(add_num(3, NA))
  expect_error(add_num(NA, 3))
})
