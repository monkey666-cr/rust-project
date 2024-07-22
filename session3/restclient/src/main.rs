// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     const URL: &str = "https://www.baidu.com/";

//     let response = reqwest::get(URL).await?;

//     println!("{:?}", response.text().await);

//     Ok(())
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse()?);
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9".parse()?);
    headers.insert("Cache-Control", "max-age=0".parse()?);
    headers.insert("Connection", "keep-alive".parse()?);
    headers.insert("Cookie", "BIDUPSID=E08CF5C784DA95BCAD5940BC5BC42714; PSTM=1692438466; BAIDUID=E08CF5C784DA95BC718AE7C65043C08F:FG=1; BAIDUID_BFESS=E08CF5C784DA95BC718AE7C65043C08F:FG=1; BDUSS=pnNmhzdURkaXI2Z2NHUXplSFN5dm1oY2VwWnloSWMzS2xENEVaVHZweE9ESk5rSVFBQUFBJCQAAAAAAAAAAAEAAABqdkI60KGzwrXEzt7EzgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE5%7Ea2ROf2tkVm; BDUSS_BFESS=pnNmhzdURkaXI2Z2NHUXplSFN5dm1oY2VwWnloSWMzS2xENEVaVHZweE9ESk5rSVFBQUFBJCQAAAAAAAAAAAEAAABqdkI60KGzwrXEzt7EzgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE5%7Ea2ROf2tkVm; COOKIE_SESSION=35_0_3_3_2_18_1_0_3_3_5_0_59_0_38_0_1709960004_0_1709959966%7C3%230_0_1709959966%7C1; Hm_lvt_aec699bb6442ba076c8981c6dc490771=1716217191; H_PS_PSSID=60470_60443_60492_60498_60474; BD_UPN=123353; BA_HECTOR=ah8lak8i0h058k2g200lalahanue3u1j9skb41u; ZFY=niM6jYAQENYCgRGrdtQoc:Aki:AjeMEeUAw1IlrgLJF9Y:C; H_PS_PSSID=60470_60443_60492_60498_60474".parse()?);
    headers.insert("Sec-Fetch-Dest", "document".parse()?);
    headers.insert("Sec-Fetch-Mode", "navigate".parse()?);
    headers.insert("Sec-Fetch-Site", "none".parse()?);
    headers.insert("Sec-Fetch-User", "?1".parse()?);
    headers.insert("Upgrade-Insecure-Requests", "1".parse()?);
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".parse()?);
    headers.insert(
        "sec-ch-ua",
        "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\", \"Google Chrome\";v=\"126\"".parse()?,
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse()?);
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse()?);

    let request = client
        .request(reqwest::Method::GET, "https://www.baidu.com/")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
