test_that("bndm_matching()", {
  expect_identical(bndm_matching("aaa", "bbbaaabbbaaabbb"), c(3, 9))
  expect_identical(bndm_matching("aaa", "bbbccccccbbbcccccccbbb"), numeric(0))
})
