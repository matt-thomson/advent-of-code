# frozen_string_literal: true

module AdventOfCode2020
  module Day08
    class Problem
      def initialize(path)
        @instructions = File
                        .readlines(path)
                        .map(&:split)
                        .map { |instruction, count| [instruction, count.to_i] }
      end

      def part_one
        counter = 0
        accumulator = 0
        visited = Set.new

        until visited.include?(counter)
          visited << counter
          operation, argument = @instructions.fetch(counter)

          case operation
          when 'acc'
            accumulator += argument
            counter += 1
          when 'jmp'
            counter += argument
          when 'nop'
            counter += 1
          end
        end

        accumulator
      end
    end
  end
end
