defmodule Day04.RoomTest do
  use ExUnit.Case

  alias Day04.Room

  test "parses a room" do
    input = "def-fed-abc-123[vwxyz]"
    room = input |> Room.parse

    assert room.words == ["def", "fed", "abc"]
    assert room.sector_id == 123
    assert room.checksum == "vwxyz"
  end

  test "calculates the checksum for a room" do
    input = "def-fed-abc-123[vwxyz]"
    checksum = input |> Room.parse |> Room.checksum

    assert checksum == "defab"
  end

  test "determines whether a room is valid" do
    valid   = "def-fed-abc-123[defab]"
    invalid = "def-fed-abc-123[defba]"

    assert   valid |> Room.parse |> Room.valid?
    refute invalid |> Room.parse |> Room.valid?
  end

  test "decrypts a room correctly" do
    input = "qzmt-zixmtkozy-ivhz-343[zimth]"
    decrypted = input |> Room.parse |> Room.decrypt

    assert decrypted == "very encrypted name"
  end
end
