package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPacket(t *testing.T) {
	p := packet{line: decode("D2FE28")}
	assert.Equal(t, 6, p.version())
	assert.Equal(t, 4, p.id())
	assert.Equal(t, 2021, p.val())
}

func TestPacketTwo(t *testing.T) {
	p := packet{line: decode("38006F45291200")}
	assert.Equal(t, 1, p.version())
	assert.Equal(t, 6, p.id())
	assert.Equal(t, 0, p.ltid())
	assert.Equal(t, 27, p.len())
	// assert.Equal(t, 2021, p.val())
}
