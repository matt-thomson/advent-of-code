# frozen_string_literal: true

module AdventOfCode2020
  module Day02
    class Password
      INPUT_REGEX = /\A(?<min>\d+)-(?<max>\d+) (?<char>[a-z]): (?<password>[a-z]+)\Z/.freeze

      def initialize(input)
        matches = input.match(INPUT_REGEX)

        @min = matches[:min].to_i
        @max = matches[:max].to_i
        @char = matches[:char]
        @password = matches[:password]
      end

      def valid?
        (@min..@max).cover?(@password.count(@char))
      end
    end
  end
end
