test_that("mul_num()", {
  expect_identical(mul_num(5, 5), 25.0)
  expect_identical(mul_num(3, 3), 9.0)
  expect_identical(mul_num(6, 5), 30.0)
  expect_identical(mul_num(10, 5), 50.0)
  expect_error(mul_num(3, NA))
  expect_error(mul_num(NA, 3))
})
