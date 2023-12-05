export def main [
	name: string@"data" = "test" # on what data to run test or input
] {
	let input = ( open $"data/($name)2.txt" )

	fun $input
	
}

def data [] {
	["test" "input"]
}

def fun [input: string] {
	print $input
}

use std assert
#[test]
def test_data [] {
	let input = ( open data/test2.txt )

	assert equal (fun $input) "e"
}
