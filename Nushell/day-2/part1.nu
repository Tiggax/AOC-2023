use std log

export def main [] {
	let input = ( open data/input1.txt )

	print (fun $input)
	
}

export def fun [input: string] {
	$input 
	| lines 
	| each {|line|
		read_line $line
	}
	| where result
	| reduce -f 0 {|it, acc|
		$acc + $it.id
	}
}

def read_line [input: string] {
	let data = ($input 
		| parse -r 'Game (?<ID>\d+):(?<sets>.*)' 
		| get 0)
	let game = $data 
	| get sets 
	| split row ";"
	| each {|set|
		$set 
		| split row ","
		| each {|col|
			let pair = ($col 
				| str trim 
				| split row " "
			)	
			check $pair
		}
	}
	| flatten
	$game
	if false in $game {
		return {id: ($data | get id | into int) result: false}
	}
	return {id: ($data | get id | into int) result: true}
}

def check [pair] {
	let colour = $pair | get 1
	let num = $pair | get 0 | into int
	
	match $colour {
		"blue" => {
			if $num > 14 { false } else {true}
		}
		"green" => {
			if $num > 13 { false } else {true}
		}
		"red" => {
			if $num > 12 { false } else {true}
		}
	}
}


use std assert
#[test]
def test_data [] {
	let input = ( open data/test1.txt )

	assert equal (fun $input) "e"
}
