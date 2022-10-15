test_that("mod_num()", {
  expect_identical(mod_num(10, 5), 0.0)
  expect_identical(mod_num(3, 3), 0.0)
  expect_identical(mod_num(12, 3), 0.0)
  expect_identical(mod_num(100, 50), 0.0)
  expect_error(mod_num(3, NA))
  expect_error(mod_num(NA, 3))
})