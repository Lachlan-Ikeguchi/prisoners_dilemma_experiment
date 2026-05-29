-- name: PAVPLOV
-- Win-stay lose-shift strategy

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- Get my previous move (approximate by looking at what opponent would have seen)
    -- Since we don't have our own history, we use opponent's last move
    -- This is a simplified Pavlov that just mirrors
    return history[#history]
end
