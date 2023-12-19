# Packets
## Overview
Packets are 128-byte chunks of data that servers and clients use to communicate. For Kaiten, each packet contains all the information required for a specific request, and must fit within 128 bytes. The first byte is the type byte, which tells the receiver what kind of packet it is. Next is data that is specific to the type of the packet. The rest of the packet outside of the data range (as indicated by the type byte) can be ignored.

**Size:** 128 bytes\
**Strcture:**
- `0x00`: Type
- `0x01-0xff`: Data

## Formats
The format of the packet data may differ depending on the type of packet it is. Most packets are "**header-only**", meaning they only contain the type as the value. **Message-containing** packets contain strings that can be up to 126 characters in length, followed by a NULL (`0x00`) byte. Strings must be formatted in **valid utf-8**. Finally, the **move** format is used to communicate information about a move a player has made. Move-formatted packets contain multiple data fields at specific offsets.

**Move format:**
<table>
    <tr>
        <th>Offset</th>
        <th>Field</th>
        <th>Description</th>
        <th>Range</th>
    </tr>
    <tr>
        <td><code>0x00</code></td>
        <td>Player</td>
        <td>Which player made the move.</td>
        <td>
            <code>0x00</code>: Red<br>
            <code>0x01</code>: Black<br>
        </td>
    </tr>
    <tr>
        <td><code>0x01</code></td>
        <td>Axis</td>
        <td>Whether the player slid a row or column.</td>
        <td>
            <code>0x00</code>: Column<br>
            <code>0x01</code>: Row<br>
        </td>
    </tr>
    <tr>
        <td><code>0x02</code></td>
        <td>Position</td>
        <td>The row or column being slid by the player. <i>(Zero-indexed)</i></td>
        <td>
            <code>0x00</code> - <code>0xff</code>
        </td>
    </tr>
    <tr>
        <td><code>0x03</code></td>
        <td>Reverse</td>
        <td>The direction in which the row or column was slid.</td>
        <td>
            <code>0x00</code>: Right/Down<br>
            <code>0x01</code>: Left/Up<br>
        </td>
    </tr>
    <tr>
        <td><code>0x04</code></td>
        <td>Capture?</td>
        <td>Whether or not the player has made a capture.</td>
        <td>
            <code>0x01</code>: <b>No</b> - Player has not captured. Capture column row can be ignored.<br>
            <code>0x00</code>: <b>Yes</b> - Player has captured. Capture column row contain capture position.<br>
        </td>
    </tr>
    <tr>
        <td><code>0x05</code></td>
        <td>Capture Column</td>
        <td>The column of the captured piece.</td>
        <td>
            <code>0x00</code> - <code>0xff</code>
        </td>
    </tr>
    <tr>
        <td><code>0x06</code></td>
        <td>Capture Row</td>
        <td>The row of the captured piece.</td>
        <td>
            <code>0x00</code> - <code>0xff</code>
        </td>
    </tr>
</table>

*Offsets are relative to the type byte. I.e., offset `0x00` is byte `0x01` in the packet data.*\
*All fields are 1 byte in size.*

## Server Packet Types
| Value | Name              | Description                   | Format                |
|-------|-------------------|-------------------------------|-----------------------|
| `0x00`| Empty             | Packet contains no data.      | Header-only           |
| `0x01`| Busy*             | Server cannot take requests.  | Header-only           |
| `0x02`| Info**            | Server information.           | Message-containing    |
| `0x03`| InvalidCommand    | Invalid packet type.          | Header-only           |
| `0x04`| IllegalCommand    | Request cannot be completed.  | Header-only           |
| `0x05`| JoinDenied        | Player cannot join game.      | Header-only           |
| `0x06`| JoinAccepted      | Player successfully joined.   | Header-only           |
| `0x07`| Kick              | Player no longer in game.     | Header-only           |
| `0x08`| GameBegin         | Ready to take/echo moves.     | Header-only           |
| `0x09`| GameEnd           | Game has ended.               | Header-only           |
| `0x0A`| TurnBegin         | Ready to accept move.         | Header-only           |
| `0x0B`| TurnEnd           | No longer accepting moves.    | Header-only           |
| `0x0C`| InvalidMove       | Move packet was malformed.    | Header-only           |
| `0x0D`| IllegalMove       | Move is not allowed.          | Header-only           |
| `0x0E`| Move              | Contains opponent move.       | Move                  |
| `0x0F`| Message           | Chat / move notation.         | Message-containing    |
| `0x10`| UnknownError      | Unhandled error occured.      | Message-containing    |

*: This may be sent when the server is overloaded.\
**: Information about the server such as room name, game time, etc...

## Client Packet Types
| Value | Name              | Description                   | Format                |
|-------|-------------------|-------------------------------|-----------------------|
| `0x00`| Empty             | Packet contains no data.      | Header-only           |
| `0x01`| InfoRequest       | Client asks for server info.  | Header-only           |
| `0x02`| JoinRequest       | Player is asking to join.     | Header-only           |
| `0x03`| Leave             | Player is leaving.            | Header-only           |
| `0x04`| Move              | Contains player move.         | Move                  |
| `0x05`| InvalidCommand    | Invalid packet type.          | Header-only           |
| `0x06`| IllegalCommand    | Request cannot be completed.  | Header-only           |
| `0x07`| Message           | Chat message.                 | Message-containing    |