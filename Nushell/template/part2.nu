export def main [input: path] {
	let input = ( open data/input1.txt )

	fun $input
	
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
