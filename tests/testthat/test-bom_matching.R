test_that("bom_matching", {
  expect_identical(bom_matching("aaa", "bbbaaabbbaaabbb"), c(3, 9))
  expect_identical(bom_matching("aaa", "bbbccccccbbbcccccccbbb"), numeric(0))
})
