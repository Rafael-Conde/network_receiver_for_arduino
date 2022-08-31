use upnp_rs::ssdp::search::*;
use upnp_rs::SpecVersion;
fn main()
{
	let mut options = Options::default_for(SpecVersion::V10);
	options.search_target = SearchTarget::RootDevices;

	match search_once(options)
	{
		Ok(responses) =>
		{
			println!("search returned {} results.", responses.len());
			for (index, response) in responses.iter().enumerate()
			{
				println!("{}: {:#?}", index, response);
			}
		}
		Err(error) =>
		{
			println!("search failed with error: {:#?}", error);
		}
	}
}
