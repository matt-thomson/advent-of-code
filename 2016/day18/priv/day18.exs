filename = System.argv |> Enum.at(0)
rows = System.argv |> Enum.at(1)

IO.puts Day18.solve(filename, rows)
