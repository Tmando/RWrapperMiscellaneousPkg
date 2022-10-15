test_that("div_num()", {
  expect_identical(div_num(10, 5), 2.0)
  expect_identical(div_num(3, 3), 1.0)
  expect_identical(div_num(12, 3), 4.0)
  expect_identical(div_num(100, 50), 2.0)
  expect_error(div_num(3, NA))
  expect_error(div_num(NA, 3))
})
