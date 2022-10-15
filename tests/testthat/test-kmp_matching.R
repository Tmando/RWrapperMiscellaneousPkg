test_that("kmp_matching()", {
  expect_equal(kmp_matching("aaa", "bbbaaabbbaaabbb"), c(3, 9))
  expect_equal(kmp_matching("aaa", "bbbccccccbbbcccccccbbb"), numeric(0))
})
