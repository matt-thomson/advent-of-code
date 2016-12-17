defmodule Day14.Hash do
  @moduledoc """
  Represents the result of applying the hash function.
  """
  alias Day14.Hash

  defstruct [:index, :triple, :fives]

  def new(index, salt) do
    hash = :md5 |> :crypto.hash("#{salt}#{index}") |> Base.encode16
    triple = hash |> find_groups(3) |> Enum.at(0)
    fives = hash |> find_groups(5) |> Enum.uniq

    %Hash{index: index, triple: triple, fives: fives}
  end

  def index(%Hash{index: index}),    do: index
  def triple(%Hash{triple: triple}), do: triple
  def fives(%Hash{fives: fives}),    do: fives

  def has_five?(%Hash{fives: fives}, char), do: fives |> Enum.member?(char)

  defp find_groups(input, length) do
    input
    |> to_char_list
    |> Enum.chunk_by(&(&1))
    |> Enum.filter(&(Enum.count(&1) >= length))
    |> Enum.map(&(Enum.at(&1, 0)))
  end
end
