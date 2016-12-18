defmodule Day17.Position do
  @moduledoc """
  Represents a state as part of a solution.
  """
  alias Day17.Directions
  alias Day17.Position

  defstruct [:input, :coordinates, :directions]

  @coordinates 0..3
  @directions [:up, :down, :left, :right]

  def new(input) do
    %Position{input: input, coordinates: {0, 0}, directions: []}
  end

  def step(position, direction) do
    coordinates = position.coordinates |> move(direction)
    directions = position.directions ++ [direction]

    %{position | coordinates: coordinates, directions: directions}
  end

  def valid_directions(position) do
    full_input =
      "#{position.input}#{position.directions |> Directions.to_string}"

    :md5
    |> :crypto.hash(full_input)
    |> Base.encode16
    |> to_char_list
    |> Enum.take(4)
    |> Enum.zip(@directions)
    |> Enum.filter(&valid_direction?/1)
    |> Enum.map(fn ({_, direction}) -> direction end)
    |> Enum.filter(&(can_move?(position.coordinates, &1)))
  end

  defp valid_direction?({?B, _}), do: true
  defp valid_direction?({?C, _}), do: true
  defp valid_direction?({?D, _}), do: true
  defp valid_direction?({?E, _}), do: true
  defp valid_direction?({?F, _}), do: true
  defp valid_direction?({_ , _}), do: false

  defp can_move?(coordinates, direction) do
    {row, column} = coordinates |> move(direction)
    @coordinates |> Enum.member?(row) && @coordinates |> Enum.member?(column)
  end

  defp move({row, column}, :up)   , do: {row - 1, column}
  defp move({row, column}, :down) , do: {row + 1, column}
  defp move({row, column}, :left) , do: {row, column - 1}
  defp move({row, column}, :right), do: {row, column + 1}
end
