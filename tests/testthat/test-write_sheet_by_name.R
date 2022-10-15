test_that("write sheet by name", {
  expect_equal(write_sheet_by_name("write_sheet_by_name/out_file_one.xlsx","Res 1",jsonlite::toJSON(mtcars)),TRUE)
})
