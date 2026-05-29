-- name: GAMBLER
-- random strategy with probability based on round parity

math.randomseed(os.time())

function decide(round, history)
    local prob = 0.7
    if round % 2 == 0 then prob = 0.3 end
    
    if math.random() < prob then
        return "cooperate"
    else
        return "defect"
    end
end
