test_that("Read excel sheet by name", {
  sheet_output <- read_sheet_by_name("read_sheet_by_name/example_input.xlsx", "Test1")
  out_df <- jsonlite::fromJSON(sheet_output)
  expect_equal(names(out_df),c("am","carb","cyl","disp","drat","gear","hp","mpg","qsec","vs","wt"))
})
