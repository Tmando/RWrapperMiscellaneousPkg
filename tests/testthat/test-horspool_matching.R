test_that("horspool_matching()", {
  expect_equal(horspool_matching("aaa", "bbbaaabbbaaabbb"), c(3, 9))
  expect_equal(horspool_matching("aaa", "bbbccccccbbbcccccccbbb"), numeric(0))
})
