fn main() -> Result<(), ureq::Error> {
  let body: String = ureq::get("http://192.168.1.144:5045/get-bills")
      .set("Example-Header", "header value")
      .call()?
      .into_string()?;
  println!("{}", body);
  Ok(())
}