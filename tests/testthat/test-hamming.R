test_that("hamming()", {
  expect_equal(hamming("Baum", "Haus"), 2)
  expect_equal(hamming("00110", "00100"), 1)
  expect_equal(hamming("12345", "13344"), 2)
  expect_equal(hamming("44555", "44666"), 3)
  expect_equal(hamming("43555", "44666"), 4)
})
