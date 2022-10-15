test_that("levenstein()", {
  expect_equal(levenshtein("00110", "00100"), 1)
  expect_equal(levenshtein("10110", "00100"), 2)
  expect_equal(levenshtein("11110", "00100"), 3)
  expect_equal(levenshtein("11110", "00101"), 4)
})
