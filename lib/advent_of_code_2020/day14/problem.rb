# frozen_string_literal: true

require_relative 'mask'

module AdventOfCode2020
  module Day14
    class Problem
      MASK_REGEX = /\Amask = (?<mask>[01X]+)\Z/.freeze
      MEM_REGEX = /\Amem\[(?<address>\d+)\] = (?<value>\d+)\Z/.freeze

      def initialize(path)
        @instructions = File.readlines(path).map { |line| parse_instruction(line.strip) }
      end

      def part_one
        memory = {}
        mask = nil

        @instructions.each do |instruction|
          case instruction[0]
          when :mask then mask = instruction[1]
          when :mem then memory[instruction[1]] = mask.apply_v1(instruction[2])
          end
        end

        memory.values.sum
      end

      def part_two
        memory = {}
        mask = nil

        @instructions.each do |instruction|
          case instruction[0]
          when :mask then mask = instruction[1]
          when :mem
            mask.apply_v2(instruction[1]).each { |address| memory[address] = instruction[2] }
          end
        end

        memory.values.sum
      end

      private

      def parse_instruction(line)
        if (mask_matches = line.match(MASK_REGEX))
          [:mask, Mask.new(mask_matches[:mask])]
        elsif (mem_matches = line.match(MEM_REGEX))
          [:mem, mem_matches[:address].to_i, mem_matches[:value].to_i]
        end
      end
    end
  end
end
