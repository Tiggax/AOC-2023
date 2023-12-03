use std log

export def main [] {
	let input = ( open data/input1.txt )

	print (fun $input)
	
}

export def fun [input: string] {
	$input 
	| lines 
	| parse -r 'Game (?<ID>\d+):(?<sets>.*)' 
	| each {|line|
		read_line $line
	}
	| reduce -f 0 {|it, acc|
		($it 
		| values 
		| reduce -f 1 {|a,b| 
			$a * $b
		}) + $acc
	}
}

def read_line [input] {
	mut max = {
		"blue": 0,
		"green": 0,
		"red": 0,
	}

	
	let game = ($input 
		| get sets 
		| split row ";"
	)

	for set in $game {
		let set = $set
		| split row ","

		for col in $set {
			let pair = ($col
			| str trim
			| split row " ")

			let colour = $pair | get 1
			let num = $pair | get 0 | into int
	
			match $colour {
				"blue" => {
					if $num > $max.blue { $max.blue = $num  }
				}
				"green" => {
					if $num > $max.green { $max.green = $num }
				}
				"red" => {
					if $num > $max.red { $max.red = $num }
				}
			}
		}
	}
	let out = $max
	$out
}

export def check [max pair] {
	
	mut val = $max
	let colour = $pair | get 1
	let num = $pair | get 0 | into int
	
	match $colour {
		"blue" => {
			if $num > $val.blue { $val.blue = $num  }
		}
		"green" => {
			if $num > $val.green { $val = ($val | upsert "green" $num) }
		}
		"red" => {
			if $num > $val.red { $val = ($val | upsert "red" $num) }
		}
	}

	$val
}
