# frozen_string_literal: true

module AdventOfCode2020
  module Day06
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        File.read(@path).split("\n\n").sum do |group|
          group.gsub(/\s/, '').chars.uniq.count
        end
      end
    end
  end
end
