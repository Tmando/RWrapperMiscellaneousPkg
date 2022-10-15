test_that("resize images", {
  expect_equal(resize_image('resize_images/20220824_214140.jpg',500,500,'Nearest','resize_images/20220824_214140_nearest.jpg',TRUE),TRUE)
  expect_equal(resize_image('resize_images/20220824_214212.jpg',500,500,'Triangle','resize_images/20220824_214212_triangle.jpg',TRUE),TRUE)
  expect_equal(resize_image('resize_images/20220824_214215.jpg',500,500,'CatmullRom','resize_images/20220824_214215_catmullRom.jpg',TRUE),TRUE)
  expect_equal(resize_image('resize_images/20220824_214219.jpg',500,500,'Gaussian','resize_images/20220824_214219_gaussian.jpg',TRUE),TRUE)
  expect_equal(resize_image('resize_images/20220824_214217.jpg',500,500,'Lanczos3','resize_images/20220824_214217_lanczos3.jpg',TRUE),TRUE)
})
