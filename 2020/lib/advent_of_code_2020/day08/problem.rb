# frozen_string_literal: true

require_relative 'computer'

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
        computer = Computer.new(@instructions)
        computer.run!

        computer.accumulator
      end

      def part_two
        (0...@instructions.length).each do |index|
          instructions = modified_instructions(index)
          next if instructions.nil?

          computer = Computer.new(instructions)
          computer.run!

          break computer.accumulator if computer.counter == @instructions.length
        end
      end

      private

      def modified_instructions(index)
        return if @instructions[index].first == 'acc'

        @instructions.each_with_index.map do |instruction, i|
          i == index ? modified_instruction(instruction) : instruction
        end
      end

      def modified_instruction(instruction)
        new_instruction = instruction.dup
        new_operation = instruction[0] == 'nop' ? 'jmp' : 'nop'

        new_instruction[0] = new_operation
        new_instruction
      end
    end
  end
end
