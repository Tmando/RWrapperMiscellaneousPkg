test_that("crypto Test", {
  expect_equal(get_sha224("Hello World"), "c4890faffdb0105d991a461e668e276685401b02eab1ef4372795047")
  expect_equal(get_sha256("Hello World"), "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e")
  expect_equal(get_sha384("Hello World"), "99514329186b2f6ae4a1329e7ee6c610a729636335174ac6b740f9028396fcc803d0e93863a7c3d90f86beee782f4f3f")
  expect_equal(get_sha512("Hello World"), "2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f27e853d8585719e0e67cbda0daa8f51671064615d645ae27acb15bfb1447f459b")
  expect_equal(get_sha1("Hello World"), "0a4d55a8d778e5022fab701977c5d840bbc486d0")
})
