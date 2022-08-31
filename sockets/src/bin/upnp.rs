
use upnp_rs::{SpecVersion, discovery::search::{SearchTarget, Options, search_once}};



fn main()
{
	let mut options = Options::default_for(SpecVersion::V10);
	options.search_target = SearchTarget::RootDevice;

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
