// Package gigasecond does only one thing.
package gigasecond

import "time"

// AddGigasecond adds a gigasecond to times
func AddGigasecond(t time.Time) time.Time {
	return t.Add(time.Second * 1000000000)
}
