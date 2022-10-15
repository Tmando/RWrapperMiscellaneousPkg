test_that("levenstein_smid()", {
  expect_equal(levenshtein_smid("00110", "00100"), 1)
  expect_equal(levenshtein_smid("10110", "00100"), 2)
  expect_equal(levenshtein_smid("11110", "00100"), 3)
  expect_equal(levenshtein_smid("11110", "00101"), 4)
})
