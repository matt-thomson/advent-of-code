input = System.argv |> Enum.at(0)
length = System.argv |> Enum.at(1)

IO.puts Day16.solve(input, length)
