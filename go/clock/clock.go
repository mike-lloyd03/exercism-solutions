package clock

import (
	"fmt"
	"time"
)

type Clock struct {
	time time.Time
}

func New(hour, minute int) Clock {
	m_quotient := minute / 60
	m_remainder := minute % 60
	h_remainder := (hour + m_quotient) % 24

	if m_remainder < 0 {
		m_remainder = 60 + m_remainder
		h_remainder -= 1
	}

	if h_remainder < 0 {
		h_remainder = 24 + h_remainder
	}

	return Clock{time.Date(2000, 1, 1, h_remainder, m_remainder, 0, 0, time.UTC)}
}

func (c Clock) String() string {
	return c.time.Format("15:04")
}

func (c Clock) Add(minutes int) Clock {
	d, err := time.ParseDuration(fmt.Sprintf("%dm", minutes))
	if err != nil {
		fmt.Println("Could not parse '{}' as an integer.", minutes)
	}
	newTime := c.time.Add(d)
	return New(newTime.Hour(), newTime.Minute())
}

func (c Clock) Subtract(minutes int) Clock {
	return c.Add(-minutes)
}
