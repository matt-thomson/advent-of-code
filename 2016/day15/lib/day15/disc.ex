defmodule Day15.Disc do
  @moduledoc """
  Represents a disc in the sculpture.
  """
  alias Day15.Disc

  defstruct [:number, :positions, :start]

  def parse(input) do
    input = input |> String.strip
    [_, number, positions, start] = regex |> Regex.run(input)

    %Disc{
      number: number |> parse_int,
      positions: positions |> parse_int,
      start: start |> parse_int
    }
  end

  def correct_position?(disc, push) do
    position = disc.start + push + disc.number
    position = position |> rem(disc.positions)

    position == 0
  end

  defp parse_int(number) do
    {number, ""} = number |> Integer.parse
    number
  end

  defp regex do
    ~r{Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).}
  end
end
