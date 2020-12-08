# frozen_string_literal: true

module AdventOfCode2020
  module Day08
    class Computer
      def initialize(instructions)
        @instructions = instructions
        @counter = 0
        @accumulator = 0
        @visited = Set.new
      end

      def run!
        until @visited.include?(counter)
          @visited << counter
          operation, argument = @instructions.fetch(counter)
          step(operation, argument)
        end
      end

      attr_reader :counter, :accumulator

      private

      def step(operation, argument)
        case operation
        when 'acc'
          @accumulator += argument
          @counter += 1
        when 'jmp'
          @counter += argument
        when 'nop'
          @counter += 1
        end
      end
    end
  end
end
