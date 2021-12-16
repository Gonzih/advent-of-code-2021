package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPacketSimple(t *testing.T) {
	p := packet{line: decode("D2FE28")}
	assert.Equal(t, 6, p.version())
	assert.Equal(t, 4, p.id())
	val, leng := p.val()
	assert.Equal(t, 2021, val)
	assert.Equal(t, 15, leng)
}

func TestPacketTwoSubPacksLen(t *testing.T) {
	p := packet{line: decode("38006F45291200")}
	assert.Equal(t, 1, p.version())
	assert.Equal(t, 6, p.id())
	assert.Equal(t, 0, p.ltid())
	assert.Equal(t, 27, p.payloadLen())

	subps := p.subPackets()
	assert.Len(t, subps, 2)

	assert.Equal(t, 6, subps[0].version())
	assert.Equal(t, 4, subps[0].id())
	val, leng := subps[0].val()
	assert.Equal(t, 10, val)
	assert.Equal(t, 5, leng)

	assert.Equal(t, 2, subps[1].version())
	assert.Equal(t, 4, subps[1].id())
	val, leng = subps[1].val()
	assert.Equal(t, 20, val)
	assert.Equal(t, 10, leng)
}

func TestPacketThreeSubPacksCount(t *testing.T) {
	p := packet{line: decode("EE00D40C823060")}
	assert.Equal(t, 7, p.version())
	assert.Equal(t, 3, p.id())
	assert.Equal(t, 1, p.ltid())
	assert.Equal(t, 3, p.payloadCount())

	subps := p.subPackets()
	assert.Len(t, subps, 3)

	i := 0
	assert.Equal(t, 2, subps[i].version())
	assert.Equal(t, 4, subps[i].id())
	val, leng := subps[i].val()
	assert.Equal(t, 1, val)
	assert.Equal(t, 5, leng)

	i++
	assert.Equal(t, 4, subps[i].version())
	assert.Equal(t, 4, subps[i].id())
	val, leng = subps[i].val()
	assert.Equal(t, 2, val)
	assert.Equal(t, 5, leng)

	i++
	assert.Equal(t, 1, subps[i].version())
	assert.Equal(t, 4, subps[i].id())
	val, leng = subps[i].val()
	assert.Equal(t, 3, val)
	assert.Equal(t, 5, leng)
}

func TestPacketThreeSumVersionsOne(t *testing.T) {
	p := packet{line: decode("8A004A801A8002F478")}
	assert.Equal(t, 4, p.version())
	assert.Equal(t, 2, p.id())
	assert.Equal(t, 16, p.sumVersions())
}

// func TestPacketThreeSumVersionsTwo(t *testing.T) {
// 	p := packet{line: decode("620080001611562C8802118E34")}
// 	assert.Equal(t, 3, p.version())
// 	assert.Equal(t, 0, p.id())
// 	assert.Equal(t, 12, p.sumVersions())
// }

// func TestPacketThreeSumVersionsThree(t *testing.T) {
// 	p := packet{line: decode("C0015000016115A2E0802F182340")}
// 	assert.Equal(t, 2, p.id())
// 	assert.Equal(t, 16, p.sumVersions())
// }

// func TestPacketThreeSumVersionsFour(t *testing.T) {
// 	p := packet{line: decode("A0016C880162017C3686B18A3D4780")}
// 	assert.Equal(t, 0, p.id())
// 	assert.Equal(t, 31, p.sumVersions())
// }
