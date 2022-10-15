test_that("hamming_smid()", {
  expect_equal(hamming_smid("Baum", "Haus"), 2)
  expect_equal(hamming_smid("00110", "00100"), 1)
  expect_equal(hamming_smid("12345", "13344"), 2)
  expect_equal(hamming_smid("44555", "44666"), 3)
  expect_equal(hamming_smid("43555", "44666"), 4)
})
