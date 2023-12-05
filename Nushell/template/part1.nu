export def main [
	name: string@"data" = "test" # on what data to run test or input
] {
	let input = ( open $"data/($name)1.txt" )

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
	let input = ( open data/test1.txt )

	assert equal (fun $input) "e"
}
