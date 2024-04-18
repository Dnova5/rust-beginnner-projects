
fn
main ()
{
  let mut reminder = 100;
  for bottles
	in 1. .101
	{
	  if bottles
		<100
		{
		  println !
			("{} bottles of beer on the wall,  Take one down and pass it around,{} of beer on the wall.",
			 reminder, 100 - bottles);
		  reminder = reminder - 1;
		}
	  else
		{
		  println ! ("No more bottles of beer on the wall");
		  println ! ("no more bottles of beer.");
		  println ! ("We've taken them down");
		  println ! ("and passed them around");
		  println ! ("now  we're drunk and passed out!");
		}

	}
}
