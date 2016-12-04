defmodule Day04.Room do
  @moduledoc """
  Represents a room in the Easter Bunny HQ.
  """
  alias Day04.Room

  @last_regex Regex.compile!("(\\d+)\\[([a-z]{5})\\]")

  defstruct [:words, :sector_id, :checksum]

  def parse(input) do
    {words, [last]} =
      input
      |> String.strip
      |> String.split("-")
      |> Enum.split(-1)

    {sector_id, checksum} = last |> parse_last

    %Room{words: words, sector_id: sector_id, checksum: checksum}
  end

  def checksum(%Room{words: words}) do
    words
    |> Enum.flat_map(&String.graphemes/1)
    |> Enum.group_by(&(&1))
    |> Enum.map(fn {c, cs} -> {c, cs |> Enum.count} end)
    |> Enum.sort(&order_character_pair/2)
    |> Enum.take(5)
    |> Enum.map(fn {c, _} -> c end)
    |> Enum.join
  end

  def valid?(room = %Room{checksum: checksum}) do
    checksum(room) == checksum
  end

  def decrypt(%Room{words: words, sector_id: sector_id}) do
    words
    |> Enum.map(&(decrypt_word(&1, sector_id)))
    |> Enum.join(" ")
  end

  defp parse_last(last) do
    [sector_id_str, checksum] =
      @last_regex
      |> Regex.run(last, capture: :all_but_first)

    {sector_id, ""} = sector_id_str |> Integer.parse

    {sector_id, checksum}
  end

  defp order_character_pair({c1, count1}, {c2, count2}) do
    cond do
      count1 < count2 -> false
      count1 > count2 -> true
      c1 <= c2 -> true
      c1 >  c2 -> false
    end
  end

  defp decrypt_word(word, sector_id) do
    word
    |> to_char_list
    |> Enum.map(&(decrypt_character(&1, sector_id)))
    |> to_string
  end

  defp decrypt_character(char, sector_id) do
    char
    |> (fn (x) -> x - ?a end).()
    |> (fn (x) -> x + sector_id end).()
    |> rem(26)
    |> (fn (x) -> x + ?a end).()
  end
end
