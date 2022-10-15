test_that("add_num()", {
  expect_identical(sub_num(3, 4), -1.0)
  expect_identical(sub_num(1, 2), -1.0)
  expect_identical(sub_num(4, 5), -1.0)
  expect_identical(sub_num(10, 5), 5.0)
  expect_error(sub_num(3, NA))
  expect_error(sub_num(NA, 3))
})
